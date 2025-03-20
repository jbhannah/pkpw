import { copyPassword } from "../util";
import { Button } from "./Button";
import { PasswordSignalProps } from "./PasswordBox";

export const PasswordCopyButton = ({
  password,
}: Readonly<PasswordSignalProps>) => (
  <Button classes="btn-accent" onClick={() => copyPassword(password)}>
    Copy
  </Button>
);
