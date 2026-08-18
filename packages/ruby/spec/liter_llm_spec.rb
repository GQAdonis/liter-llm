# frozen_string_literal: true

require_relative "../lib/liter_llm"

RSpec.describe LiterLlm do
  # No generated function is safe to call with no arguments, so this exercises the
  # binding through the generated `DocumentContent` class instead: the `require_relative`
  # above dlopens the compiled extension (LoadError when missing), the keyword
  # constructor registered by Magnus is invoked, and every field is read back through its
  # generated accessor. A dropped or renamed field fails here, because the constructor
  # ignores unknown keys and the accessor would return the field's default instead of the
  # value passed in. It proves nothing beyond field storage. Create-only scaffold seed:
  # alef never regenerates over this file, so replace it with a real suite. ~keep
  it "constructs the generated `DocumentContent` class from keyword arguments" do
    instance = described_class::DocumentContent.new(data: "alef-scaffold", media_type: "alef-scaffold")
    expect([instance.data, instance.media_type]).to eq(["alef-scaffold", "alef-scaffold"])
  end
end
