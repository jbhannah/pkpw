import { ComponentProps } from "preact";

interface ButtonProps extends ComponentProps<"button"> {
  classes?: string;
  type?: "button" | "submit" | "reset";
}

export function Button({ classes, type, ...props }: Readonly<ButtonProps>) {
  return <button type={type ?? "button"} class={`btn ${classes}`} {...props} />;
}
