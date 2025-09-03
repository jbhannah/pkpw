import { copyPassword } from "../util";
import Button from "./Button";
import type { PasswordSignalProps } from "./PasswordBox";

const PasswordCopyButton = ({ password }: Readonly<PasswordSignalProps>) => (
  <Button classes="btn-accent" onClick={() => copyPassword(password.value)}>
    Copy
  </Button>
);

export default PasswordCopyButton;
