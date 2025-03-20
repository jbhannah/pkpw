import { generatePassword } from "../util";
import { Button } from "./Button";
import { PasswordSignalProps } from "./PasswordBox";

export const PasswordGenerateButton = ({
  password,
}: Readonly<PasswordSignalProps>) => (
  <Button classes="btn-primary" onClick={() => generatePassword(password)}>
    Generate
  </Button>
);
