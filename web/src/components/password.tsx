import { useSignal } from "@preact/signals";
import { generate_password as pkpw } from "pkpw";

export function Password() {
  const password = useSignal(pkpw());

  return (
    <div class="w-full flex flex-col items-center gap-y-4">
      <label class="w-full">
        <span class="sr-only">Generated password</span>
        <input
          class="w-full input input-xl text-center"
          name="password"
          type="text"
          value={password}
          readonly
        />
      </label>
      <div class="flex gap-x-4">
        <button
          type="button"
          class="btn btn-accent"
          onClick={() => navigator.clipboard.writeText(password.value)}
        >
          Copy
        </button>
        <button
          type="button"
          class="btn btn-primary"
          onClick={() => (password.value = pkpw())}
        >
          Generate
        </button>
      </div>
    </div>
  );
}
