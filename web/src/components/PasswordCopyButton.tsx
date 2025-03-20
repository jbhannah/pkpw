import { copyPassword } from "../util";
import { Button } from "./Button";
import { PasswordSignalProps } from "./PasswordBox";

export function PasswordCopyButton({
  password,
}: Readonly<PasswordSignalProps>) {
  return (
    <Button classes="btn-accent" onClick={() => copyPassword(password)}>
      Copy
    </Button>
  );
}
