// VS-BRAX ABRASAX BUS — C# named-pipe telemetry bridge (agent harness, host side)
// PRIMAL_HEX: 4f5349524953424c58434b
// Relays hex-encoded wire payloads between the VS-BRAX agent host (TS) and the
// ABRASAX mesh through the audited pipe \\.\pipe\abrasax_bus.
// Build: dotnet build (net10.0) | Run: dotnet abrasax_bus.dll
using System;
using System.IO.Pipes;
using System.Text;
using System.Threading.Tasks;

namespace VsBrax;

public static class AbrasaxBus
{
    private const string PipeName = "abrasax_bus";
    private const int BufferSize = 8192;

    /// <summary>Connect to the ABRASAX bus and relay every frame to stdout (picked up by the TS host).</summary>
    public static async Task<int> Main(string[] args)
    {
        var delay = args.Length > 0 ? int.Parse(args[0]) : 250;
        while (true)
        {
            try
            {
                using var pipe = new NamedPipeClientStream(".", PipeName, PipeDirection.InOut);
                await pipe.ConnectAsync(1000);
                var buffer = new byte[BufferSize];
                while (true)
                {
                    var read = await pipe.ReadAsync(buffer);
                    if (read <= 0)
                    {
                        break;
                    }
                    var payload = Encoding.UTF8.GetString(buffer, 0, read);
                    Console.Out.WriteLine(payload);   // TS host streams this into the right-side terminal
                    Console.Out.Flush();
                }
            }
            catch (TimeoutException)
            {
                // bus not up yet — retry
            }
            catch (Exception e)
            {
                Console.Error.WriteLine($"[vsbrax-bus] {e.Message}");
            }
            await Task.Delay(delay);
        }
    }
}
