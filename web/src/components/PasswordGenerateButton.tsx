import { generatePassword } from "../util";
import Button from "./Button";
import type { PasswordSignalProps } from "./PasswordBox";

const PasswordGenerateButton = ({
  password,
}: Readonly<PasswordSignalProps>) => (
  <Button classes="btn-primary" onClick={() => generatePassword(password)}>
    Generate
  </Button>
);

export default PasswordGenerateButton;
