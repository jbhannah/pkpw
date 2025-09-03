import { type Signal, useSignal } from "@preact/signals";
import { useEffect } from "preact/hooks";
import { generatePassword } from "../util";
import PasswordCopyButton from "./PasswordCopyButton";
import PasswordField from "./PasswordField";
import PasswordGenerateButton from "./PasswordGenerateButton";
import PasswordOptions from "./PasswordOptions";
import PasswordOptionsButton from "./PasswordOptionsButton";

export interface PasswordSignalProps {
  password: Signal<string>;
}

export interface PasswordOptionsVisibleProps {
  optionsVisible: Signal<boolean>;
}

export interface PasswordOptionProps {
  len: Signal<number>;
  count: Signal<number>;
  separator: Signal<PasswordSeparator>;
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
  const optionsVisible = useSignal(false);

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
            len: len,
            count: count,
            separator: separator,
          }}
        />
        <PasswordOptionsButton optionsVisible={optionsVisible} />
      </div>
      {optionsVisible.value && (
        <PasswordOptions {...{ len, count, separator }} />
      )}
    </div>
  );
};

export default PasswordBox;
