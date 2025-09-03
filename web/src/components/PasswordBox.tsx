import { type Signal, useSignal } from "@preact/signals";
import { useEffect } from "preact/hooks";
import { generatePassword } from "../util";
import PasswordCopyButton from "./PasswordCopyButton";
import PasswordField from "./PasswordField";
import PasswordGenerateButton from "./PasswordGenerateButton";

export interface PasswordSignalProps {
  password: Signal<string>;
}

export interface PasswordOptionProps {
  len: number;
  count: number;
  separator: string;
}

export enum PasswordSeparator {
  DIGITS = "digit",
  SPECIAL = "special",
  SPACE = " ",
}

const PasswordBox = () => {
  const password = useSignal("");
  const len = useSignal(0);
  const count = useSignal(4);
  const separator = useSignal(PasswordSeparator.SPACE);

  useEffect(
    () => generatePassword(password, len.value, count.value, separator.value),
    [password, len, count, separator],
  );

  return (
    <div class="w-full flex flex-col items-center gap-y-4">
      <PasswordField password={password} />
      <div class="flex gap-x-4">
        <PasswordCopyButton password={password} />
        <PasswordGenerateButton
          {...{
            password,
            len: len.value,
            count: count.value,
            separator: separator.value,
          }}
        />
      </div>
    </div>
  );
};

export default PasswordBox;
