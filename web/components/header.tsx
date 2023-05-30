import { ClockCountdown, Cube, IconContext, UsersThree } from '@phosphor-icons/react'
import Link from 'next/link'
import * as NavigationMenu from '@radix-ui/react-navigation-menu';

export default () => {
  return (
    <IconContext.Provider
      value={{
        size: "1em",
        weight: "bold",
        className: "inline mb-1",
      }}
    >
      <header className="p-1 bg-gray-100 text-gray-800">
        <NavigationMenu.Root>
          <NavigationMenu.List asChild>
            <nav className="container mx-auto flex flex-row text-lg">
              <NavigationMenu.Item asChild>
                <h1 className="text-xl py-2">
                  <NavigationMenu.Link asChild>
                    <Link href="/">Beacon explorer</Link>
                  </NavigationMenu.Link>
                </h1>
              </NavigationMenu.Item>
              <ol className="flex flex-row flex-grow justify-end">
                <NavigationMenu.Item asChild>
                  <li className="p-2">
                    <NavigationMenu.Link asChild>
                      <Link href="/epochs"><ClockCountdown /> Epochs</Link>
                    </NavigationMenu.Link>
                  </li>
                </NavigationMenu.Item>
                <NavigationMenu.Item asChild>
                  <li className="p-2">
                    <NavigationMenu.Link asChild>
                      <Link href="/blocks"><Cube /> Blocks</Link>
                    </NavigationMenu.Link>
                  </li>
                </NavigationMenu.Item>
                <NavigationMenu.Item asChild>
                  <li className="p-2 relative">
                    <NavigationMenu.Trigger asChild>
                      <NavigationMenu.Link asChild>
                        <Link href="/validators"><UsersThree /> Validators</Link>
                      </NavigationMenu.Link>
                    </NavigationMenu.Trigger>
                    <NavigationMenu.Content className="absolute top-10 left-0 bg-white rounded shadow">
                      <NavigationMenu.Sub>
                        <NavigationMenu.List asChild>
                          <ol>
                            <NavigationMenu.Item asChild>
                              <li className="p-2">
                                <NavigationMenu.Link asChild>
                                  <Link href="/deposits">Deposits</Link>
                                </NavigationMenu.Link>
                              </li>
                            </NavigationMenu.Item>
                          </ol>
                        </NavigationMenu.List>
                      </NavigationMenu.Sub>
                    </NavigationMenu.Content>

                  </li>
                </NavigationMenu.Item>
              </ol>
            </nav>
          </NavigationMenu.List>
        </NavigationMenu.Root>
      </header>
    </IconContext.Provider >
  )
}