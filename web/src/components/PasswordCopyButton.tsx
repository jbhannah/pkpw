import { copyPassword } from "../util";
import { Button } from "./Button";
import { PasswordSignalProps } from "./Password";

export function PasswordCopyButton({
  password,
}: Readonly<PasswordSignalProps>) {
  return (
    <Button classes="btn-accent" onClick={() => copyPassword(password)}>
      Copy
    </Button>
  );
}
