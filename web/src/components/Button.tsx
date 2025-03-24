import type { ComponentProps } from "preact";

interface ButtonProps extends ComponentProps<"button"> {
  classes?: string;
  type?: "button" | "submit" | "reset";
}

const Button = ({ classes, type, ...props }: Readonly<ButtonProps>) => (
  <button type={type ?? "button"} class={`btn ${classes}`} {...props} />
);

export default Button;
