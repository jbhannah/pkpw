import { toggleOptions } from "../util";
import Button from "./Button";
import type { PasswordOptionsVisibleProps } from "./PasswordBox";

const PasswordOptionsButton = ({
  optionsVisible,
}: Readonly<PasswordOptionsVisibleProps>) => (
  <Button classes="btn-primary" onClick={() => toggleOptions(optionsVisible)}>
    {optionsVisible.value ? "Hide Options" : "Show Options"}
  </Button>
);

export default PasswordOptionsButton;
