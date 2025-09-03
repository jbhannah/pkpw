import { generatePassword } from "../util";
import Button from "./Button";
import type { PasswordOptionProps, PasswordSignalProps } from "./PasswordBox";

const PasswordGenerateButton = ({
  password,
  len,
  count,
  separator,
}: Readonly<PasswordSignalProps & PasswordOptionProps>) => (
  <Button
    classes="btn-primary"
    onClick={() =>
      generatePassword(password, len.value, count.value, separator.value)
    }
  >
    Generate
  </Button>
);

export default PasswordGenerateButton;
