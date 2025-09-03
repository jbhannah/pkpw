import { type Signal, useSignal, useSignalEffect } from "@preact/signals";
import { generatePassword } from "../util";
import Button from "./Button";
import PasswordCopyButton from "./PasswordCopyButton";
import PasswordField from "./PasswordField";
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
  SPACE = " ",
  DIGITS = "digit",
  SPECIAL = "special",
}

const PasswordBox = () => {
  const password = useSignal("");
  const len = useSignal(0);
  const count = useSignal(4);
  const separator = useSignal(PasswordSeparator.SPACE);
  const optionsVisible = useSignal(false);

  const generatePasswordOnce = () =>
    generatePassword(password, len.value, count.value, separator.value);

  useSignalEffect(generatePasswordOnce);

  return (
    <div class="w-full flex flex-col items-center gap-y-4">
      <PasswordField password={password} />
      <div class="flex gap-x-4">
        <PasswordCopyButton password={password} />
        <Button classes="btn-primary" onClick={generatePasswordOnce}>
          Regenerate
        </Button>
        <PasswordOptionsButton optionsVisible={optionsVisible} />
      </div>
      {optionsVisible.value && (
        <PasswordOptions {...{ len, count, separator }} />
      )}
    </div>
  );
};

export default PasswordBox;
