package dev.kreuzberg.literllm;

import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;

/**
 * Low-level Panama FFM bindings to {@code libliter_llm_ffi}.
 *
 * <p>
 * Each {@code static final MethodHandle} corresponds to one {@code extern "C"}
 * function exported by the native library. This class is package-private and
 * not part of the public API.
 */
final class NativeMethods {

	private static final Linker LINKER = Linker.nativeLinker();
	private static final SymbolLookup LOOKUP;

	static {
		System.loadLibrary("liter_llm_ffi");
		LOOKUP = SymbolLookup.loaderLookup();
	}

	private NativeMethods() {
	}

	// ─── Client Lifecycle ─────────────────────────────────────────────────────

	/**
	 * {@code LiterLlmClient *literllm_client_new(const char *, const char *, const char *)}
	 */
	static final MethodHandle CLIENT_NEW = downcall("literllm_client_new",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code LiterLlmClient *literllm_client_new_with_config(const char *)} */
	static final MethodHandle CLIENT_NEW_WITH_CONFIG = downcall("literllm_client_new_with_config",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code void literllm_client_free(LiterLlmClient *)} */
	static final MethodHandle CLIENT_FREE = downcall("literllm_client_free",
			FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

	// ─── JSON Request/Response Methods ────────────────────────────────────────

	/** {@code char *literllm_chat(const LiterLlmClient *, const char *)} */
	static final MethodHandle CHAT = downcall("literllm_chat",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/**
	 * {@code int32_t literllm_chat_stream(const LiterLlmClient *, const char *, callback, void *)}
	 */
	static final MethodHandle CHAT_STREAM = downcall("literllm_chat_stream", FunctionDescriptor.of(ValueLayout.JAVA_INT,
			ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_embed(const LiterLlmClient *, const char *)} */
	static final MethodHandle EMBED = downcall("literllm_embed",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_list_models(const LiterLlmClient *)} */
	static final MethodHandle LIST_MODELS = downcall("literllm_list_models",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/**
	 * {@code char *literllm_image_generate(const LiterLlmClient *, const char *)}
	 */
	static final MethodHandle IMAGE_GENERATE = downcall("literllm_image_generate",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_speech(const LiterLlmClient *, const char *)} */
	static final MethodHandle SPEECH = downcall("literllm_speech",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_transcribe(const LiterLlmClient *, const char *)} */
	static final MethodHandle TRANSCRIBE = downcall("literllm_transcribe",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_moderate(const LiterLlmClient *, const char *)} */
	static final MethodHandle MODERATE = downcall("literllm_moderate",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_rerank(const LiterLlmClient *, const char *)} */
	static final MethodHandle RERANK = downcall("literllm_rerank",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_search(const LiterLlmClient *, const char *)} */
	static final MethodHandle SEARCH = downcall("literllm_search",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_ocr(const LiterLlmClient *, const char *)} */
	static final MethodHandle OCR = downcall("literllm_ocr",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	// ─── File Operations ──────────────────────────────────────────────────────

	/** {@code char *literllm_create_file(const LiterLlmClient *, const char *)} */
	static final MethodHandle CREATE_FILE = downcall("literllm_create_file",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/**
	 * {@code char *literllm_retrieve_file(const LiterLlmClient *, const char *)}
	 */
	static final MethodHandle RETRIEVE_FILE = downcall("literllm_retrieve_file",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_delete_file(const LiterLlmClient *, const char *)} */
	static final MethodHandle DELETE_FILE = downcall("literllm_delete_file",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_list_files(const LiterLlmClient *, const char *)} */
	static final MethodHandle LIST_FILES = downcall("literllm_list_files",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_file_content(const LiterLlmClient *, const char *)} */
	static final MethodHandle FILE_CONTENT = downcall("literllm_file_content",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	// ─── Batch Operations ─────────────────────────────────────────────────────

	/** {@code char *literllm_create_batch(const LiterLlmClient *, const char *)} */
	static final MethodHandle CREATE_BATCH = downcall("literllm_create_batch",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/**
	 * {@code char *literllm_retrieve_batch(const LiterLlmClient *, const char *)}
	 */
	static final MethodHandle RETRIEVE_BATCH = downcall("literllm_retrieve_batch",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_list_batches(const LiterLlmClient *, const char *)} */
	static final MethodHandle LIST_BATCHES = downcall("literllm_list_batches",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/** {@code char *literllm_cancel_batch(const LiterLlmClient *, const char *)} */
	static final MethodHandle CANCEL_BATCH = downcall("literllm_cancel_batch",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	// ─── Response Operations ──────────────────────────────────────────────────

	/**
	 * {@code char *literllm_create_response(const LiterLlmClient *, const char *)}
	 */
	static final MethodHandle CREATE_RESPONSE = downcall("literllm_create_response",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/**
	 * {@code char *literllm_retrieve_response(const LiterLlmClient *, const char *)}
	 */
	static final MethodHandle RETRIEVE_RESPONSE = downcall("literllm_retrieve_response",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	/**
	 * {@code char *literllm_cancel_response(const LiterLlmClient *, const char *)}
	 */
	static final MethodHandle CANCEL_RESPONSE = downcall("literllm_cancel_response",
			FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	// ─── Utility ──────────────────────────────────────────────────────────────

	/** {@code double literllm_budget_usage(const LiterLlmClient *)} */
	static final MethodHandle BUDGET_USAGE = downcall("literllm_budget_usage",
			FunctionDescriptor.of(ValueLayout.JAVA_DOUBLE, ValueLayout.ADDRESS));

	/** {@code const char *literllm_last_error(void)} */
	static final MethodHandle LAST_ERROR = downcall("literllm_last_error", FunctionDescriptor.of(ValueLayout.ADDRESS));

	/** {@code void literllm_free_string(char *)} */
	static final MethodHandle FREE_STRING = downcall("literllm_free_string",
			FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

	/** {@code const char *literllm_version(void)} */
	static final MethodHandle VERSION = downcall("literllm_version", FunctionDescriptor.of(ValueLayout.ADDRESS));

	/** {@code int32_t literllm_register_provider(const char *)} */
	static final MethodHandle REGISTER_PROVIDER = downcall("literllm_register_provider",
			FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS));

	/** {@code int32_t literllm_unregister_provider(const char *)} */
	static final MethodHandle UNREGISTER_PROVIDER = downcall("literllm_unregister_provider",
			FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS));

	/**
	 * {@code int32_t literllm_set_hooks(LiterLlmClient *, const LiterLlmHookCallbacks *)}
	 */
	static final MethodHandle SET_HOOKS = downcall("literllm_set_hooks",
			FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

	// ─── Stream Callback Descriptor ───────────────────────────────────────────

	/**
	 * Function descriptor for the {@code LiterLlmStreamCallback} upcall:
	 * {@code void (*)(const char *, void *)}.
	 */
	static final FunctionDescriptor STREAM_CALLBACK_DESCRIPTOR = FunctionDescriptor.ofVoid(ValueLayout.ADDRESS,
			ValueLayout.ADDRESS);

	// ─── Helper ───────────────────────────────────────────────────────────────

	private static MethodHandle downcall(String name, FunctionDescriptor desc) {
		return LINKER.downcallHandle(LOOKUP.findOrThrow(name), desc);
	}
}
