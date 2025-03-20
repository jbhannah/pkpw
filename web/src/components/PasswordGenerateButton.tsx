import { generatePassword } from "../util";
import { Button } from "./Button";
import { PasswordSignalProps } from "./Password";

export function PasswordGenerateButton({
  password,
}: Readonly<PasswordSignalProps>) {
  return (
    <Button classes="btn-primary" onClick={() => generatePassword(password)}>
      Generate
    </Button>
  );
}
