/* VS-BRAX pipe_bridge.c — C driver: write hex wire payloads into the ABRASAX pipe.
 * PRIMAL_HEX: 4f5349524953424c58434b
 * Build (Windows): cl /LD pipe_bridge.c /Fe:pipe_bridge.dll   (MSVC)
 *                 or: gcc -shared -o pipe_bridge.dll pipe_bridge.c -lws2_32
 * Exports SendManifoldConstraint(const char* hex_payload) -> writes to \\.\pipe\abrasax
 */
#include <windows.h>
#include <stdio.h>
#include <string.h>

__declspec(dllexport) int SendManifoldConstraint(const char* hex_payload) {
    if (hex_payload == NULL || strlen(hex_payload) == 0) {
        return 1; /* empty payload */
    }
    HANDLE hPipe = CreateFile(
        TEXT("\\\\.\\pipe\\abrasax"),
        GENERIC_WRITE,
        0, NULL,
        OPEN_EXISTING,
        0, NULL);
    if (hPipe == INVALID_HANDLE_VALUE) {
        return 2; /* pipe not available */
    }
    DWORD written = 0;
    BOOL ok = WriteFile(hPipe, hex_payload, (DWORD)strlen(hex_payload), &written, NULL);
    CloseHandle(hPipe);
    return ok ? 0 : 3;
}

/* Frame-slice helper for GGUF offload chunks (raw byte pass-through). */
__declspec(dllexport) int SendFrameSlice(const unsigned char* bytes, int len) {
    if (bytes == NULL || len <= 0) {
        return 1;
    }
    HANDLE hPipe = CreateFile(
        TEXT("\\\\.\\pipe\\abrasax_frames"),
        GENERIC_WRITE,
        0, NULL,
        OPEN_EXISTING,
        0, NULL);
    if (hPipe == INVALID_HANDLE_VALUE) {
        return 2;
    }
    DWORD written = 0;
    BOOL ok = WriteFile(hPipe, bytes, (DWORD)len, &written, NULL);
    CloseHandle(hPipe);
    return ok ? 0 : 3;
}
