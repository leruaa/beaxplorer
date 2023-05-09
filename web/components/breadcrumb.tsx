import { CaretRight, House, Icon, IconContext } from "@phosphor-icons/react";
import NextLink from "next/link";
import { ReactNode, createContext, useContext } from "react";

const LinksClassNameContext = createContext<string>(null);

type LinkProps = { href: string, children: ReactNode | JSX.Element[] }

export const Link = ({ href, children }: LinkProps) => {
  const className = useContext(LinksClassNameContext);

  return (
    <>
      <CaretRight className="inline mb-1 text-gray-500" />
      <NextLink className={className} href={href}>{children}</NextLink>
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

type RootProps = { linksClassName?: string, children?: JSX.Element | JSX.Element[] }



export const Root = ({ linksClassName, children }: RootProps) => {
  return (
    <IconContext.Provider
      value={{
        size: "1em",
        weight: "bold",
        className: "inline mb-1",
      }}
    >
      <LinksClassNameContext.Provider value={linksClassName}>
        <ul className="text-lg my-4">
          <NextLink className={linksClassName} href="/"><House /></NextLink>
          {children}
        </ul>
      </LinksClassNameContext.Provider>

    </IconContext.Provider>
  );
}