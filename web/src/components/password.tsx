import { useSignal } from "@preact/signals";
import { generate_password as pkpw } from "pkpw";

export function Password() {
  const password = useSignal(pkpw());

  return (
    <div class="flex flex-col items-center gap-y-4">
      <label class="w-full max-w-lg">
        <span class="sr-only">Generated password</span>
        <input
          class="w-full input input-xl text-center"
          name="password"
          type="text"
          value={password}
          readonly
        />
      </label>
      <div>
        <button
          class="btn btn-primary"
          onClick={() => (password.value = pkpw())}
        >
          Generate
        </button>
      </div>
    </div>
  );
}
