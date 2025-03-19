import { useSignal } from "@preact/signals";
import { pkpw } from "pkpw";
import { usePostHog } from "posthog-js/react";

export function Password() {
  const posthog = usePostHog();

  const generatePassword = () => {
    posthog.capture("generated_password");
    return pkpw();
  };

  const password = useSignal(generatePassword());

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
          onClick={() => (password.value = generatePassword())}
        >
          Generate
        </button>
      </div>
    </div>
  );
}
