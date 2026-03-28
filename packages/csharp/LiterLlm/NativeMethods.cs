using System.Runtime.InteropServices;

namespace LiterLlm;

/// <summary>
/// P/Invoke declarations for the <c>libliter_llm_ffi</c> native library.
/// </summary>
/// <remarks>
/// All functions follow the C API defined in <c>liter_llm.h</c>.
/// Strings are exchanged as NUL-terminated UTF-8 C strings.
/// Returned <c>char*</c> pointers must be freed with <see cref="literllm_free_string"/>
/// unless documented otherwise (e.g. <see cref="literllm_last_error"/> and
/// <see cref="literllm_version"/>).
/// </remarks>
internal static partial class NativeMethods
{
    private const string LibName = "liter_llm_ffi";

    // ─── Client Lifecycle ────────────────────────────────────────────────────

    /// <summary>
    /// Creates a new client from a simple API-key / base-URL / model-hint triple.
    /// Returns <c>NULL</c> on failure; check <see cref="literllm_last_error"/>.
    /// The caller owns the returned pointer and must call <see cref="literllm_client_free"/>.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_client_new(IntPtr apiKey, IntPtr baseUrl, IntPtr modelHint);

    /// <summary>
    /// Creates a new client from a full JSON configuration object.
    /// Returns <c>NULL</c> on failure; check <see cref="literllm_last_error"/>.
    /// The caller owns the returned pointer and must call <see cref="literllm_client_free"/>.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_client_new_with_config(IntPtr configJson);

    /// <summary>
    /// Frees a client created by <see cref="literllm_client_new"/> or
    /// <see cref="literllm_client_new_with_config"/>. Passing <c>NULL</c> is a no-op.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void literllm_client_free(IntPtr client);

    // ─── Core API ────────────────────────────────────────────────────────────

    /// <summary>Send a chat completion request. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_chat(IntPtr client, IntPtr requestJson);

    /// <summary>Send an embedding request. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_embed(IntPtr client, IntPtr requestJson);

    /// <summary>List available models. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_list_models(IntPtr client);

    // ─── Inference API ───────────────────────────────────────────────────────

    /// <summary>Generate an image from a text prompt. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_image_generate(IntPtr client, IntPtr requestJson);

    /// <summary>Generate speech audio from text. Returns base64-encoded string or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_speech(IntPtr client, IntPtr requestJson);

    /// <summary>Transcribe audio to text. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_transcribe(IntPtr client, IntPtr requestJson);

    /// <summary>Check content against moderation policies. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_moderate(IntPtr client, IntPtr requestJson);

    /// <summary>Rerank documents by relevance. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_rerank(IntPtr client, IntPtr requestJson);

    /// <summary>Perform a web/document search. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_search(IntPtr client, IntPtr requestJson);

    /// <summary>Extract text from a document via OCR. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_ocr(IntPtr client, IntPtr requestJson);

    // ─── File Operations ─────────────────────────────────────────────────────

    /// <summary>Upload a file. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_create_file(IntPtr client, IntPtr requestJson);

    /// <summary>Retrieve file metadata by ID. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_retrieve_file(IntPtr client, IntPtr fileId);

    /// <summary>Delete a file by ID. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_delete_file(IntPtr client, IntPtr fileId);

    /// <summary>List files with optional query. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_list_files(IntPtr client, IntPtr queryJson);

    /// <summary>Retrieve raw file content as base64. Returns string or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_file_content(IntPtr client, IntPtr fileId);

    // ─── Batch Operations ────────────────────────────────────────────────────

    /// <summary>Create a new batch job. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_create_batch(IntPtr client, IntPtr requestJson);

    /// <summary>Retrieve a batch by ID. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_retrieve_batch(IntPtr client, IntPtr batchId);

    /// <summary>List batches with optional query. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_list_batches(IntPtr client, IntPtr queryJson);

    /// <summary>Cancel an in-progress batch. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_cancel_batch(IntPtr client, IntPtr batchId);

    // ─── Response Operations ─────────────────────────────────────────────────

    /// <summary>Create a new response. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_create_response(IntPtr client, IntPtr requestJson);

    /// <summary>Retrieve a response by ID. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_retrieve_response(IntPtr client, IntPtr responseId);

    /// <summary>Cancel an in-progress response. Returns JSON or <c>NULL</c>.</summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_cancel_response(IntPtr client, IntPtr responseId);

    // ─── Streaming ───────────────────────────────────────────────────────────

    /// <summary>
    /// Callback signature for streaming chat completions.
    /// <paramref name="chunkJson"/> is a NUL-terminated JSON string valid only
    /// for the duration of the callback. The callee must not free it.
    /// </summary>
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void StreamCallback(IntPtr chunkJson, IntPtr userData);

    /// <summary>
    /// Sends a streaming chat completion request. The <paramref name="callback"/>
    /// is invoked once per SSE chunk. Returns <c>0</c> on success, <c>-1</c> on failure.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int literllm_chat_stream(
        IntPtr client, IntPtr requestJson, StreamCallback callback, IntPtr userData);

    // ─── Hooks ───────────────────────────────────────────────────────────────

    /// <summary>
    /// Lifecycle hook callbacks struct passed to <see cref="literllm_set_hooks"/>.
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    internal struct LiterLlmHookCallbacks
    {
        internal IntPtr OnRequest;
        internal IntPtr OnResponse;
        internal IntPtr OnError;
        internal IntPtr UserData;
    }

    /// <summary>
    /// Registers lifecycle hook callbacks for a client. Returns <c>0</c> on success.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int literllm_set_hooks(IntPtr client, ref LiterLlmHookCallbacks callbacks);

    // ─── Utility ─────────────────────────────────────────────────────────────

    /// <summary>
    /// Returns the cumulative global spend tracked by the budget layer (in USD).
    /// Returns <c>0.0</c> if no budget is configured.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern double literllm_budget_usage(IntPtr client);

    /// <summary>
    /// Returns the last error message for the current thread, or <c>NULL</c> if
    /// no error has occurred. The caller must <b>not</b> free the returned pointer.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_last_error();

    /// <summary>
    /// Frees a string returned by any of the <c>literllm_*</c> functions that
    /// allocate response strings. Passing <c>NULL</c> is a no-op.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void literllm_free_string(IntPtr s);

    /// <summary>
    /// Returns the liter-llm library version string. The caller must <b>not</b>
    /// free the returned pointer (it is a static string).
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr literllm_version();

    /// <summary>
    /// Registers a custom LLM provider at runtime. Returns <c>0</c> on success,
    /// <c>-1</c> on failure.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int literllm_register_provider(IntPtr configJson);

    /// <summary>
    /// Unregisters a custom provider by name. Returns <c>0</c> if removed,
    /// <c>1</c> if not found, <c>-1</c> on failure.
    /// </summary>
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int literllm_unregister_provider(IntPtr name);
}
