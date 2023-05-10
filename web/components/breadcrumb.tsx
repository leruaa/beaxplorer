import { CaretRight, House, Icon, IconContext } from "@phosphor-icons/react";
import NextLink from "next/link";
import { ReactNode, createContext, useContext } from "react";
import cx from 'classnames';
import { Accent, useAccent } from "../hooks/accent";

const LinksClassNameContext = createContext<{ [key: string]: boolean }>(null);

type LinkProps = { href: string, children: ReactNode | JSX.Element[] }

export const Link = ({ href, children }: LinkProps) => {
  const className = useContext(LinksClassNameContext);

  return (
    <>
      <CaretRight className="inline mb-1 text-gray-500" />
      <NextLink className={cx(className)} href={href}>{children}</NextLink>
    </>
  );
}

type TextProps = { children: ReactNode }

export const Text = ({ children }: TextProps) => {
  return (
    <>
      <CaretRight className="inline mb-1 text-gray-500" />
      {children}
    </>
  );
}

type RootProps = { children?: ReactNode }

export const Root = ({ children }: RootProps) => {
  const accent = useAccent();
  const accentClassName = {
    "text-sky-500": accent == Accent.Sky,
    "text-indigo-500": accent == Accent.Indigo
  };

  return (
    <IconContext.Provider
      value={{
        size: "1em",
        weight: "bold",
        className: cx(accentClassName, "inline mb-1"),
      }}
    >
      <LinksClassNameContext.Provider value={accentClassName}>
        <ul className="text-lg my-4">
          <NextLink href="/"><House /></NextLink>
          {children}
        </ul>
      </LinksClassNameContext.Provider>

    </IconContext.Provider>
  );
}