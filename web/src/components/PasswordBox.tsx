import { Signal, useSignal } from "@preact/signals";
import { useEffect } from "preact/hooks";
import { generatePassword } from "../util";
import { PasswordCopyButton } from "./PasswordCopyButton";
import { PasswordField } from "./PasswordField";
import { PasswordGenerateButton } from "./PasswordGenerateButton";

export interface PasswordSignalProps {
  password: Signal<string>;
}

export function PasswordBox() {
  const password = useSignal("");

  useEffect(() => generatePassword(password), [password]);

  return (
    <div class="w-full flex flex-col items-center gap-y-4">
      <PasswordField password={password} />
      <div class="flex gap-x-4">
        <PasswordCopyButton password={password} />
        <PasswordGenerateButton password={password} />
      </div>
    </div>
  );
}
