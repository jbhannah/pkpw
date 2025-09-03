import Button from "./Button";
import { PasswordOptionsVisibleProps } from "./PasswordBox";

const PasswordOptionsButton = ({
  optionsVisible,
}: Readonly<PasswordOptionsVisibleProps>) => (
  <Button
    classes="btn-primary"
    onClick={() => (optionsVisible.value = !optionsVisible.value)}
  >
    {optionsVisible.value ? "Hide Options" : "Show Options"}
  </Button>
);

export default PasswordOptionsButton;
