import { ReactNode } from "react";
import { Link } from "react-router";

export default function BackToRoot(props: { children: ReactNode }) {
    return <Link to="/">{props.children}</Link>;
}
