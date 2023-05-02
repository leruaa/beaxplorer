import { ClockCountdown, Cube, IconContext, UsersThree } from '@phosphor-icons/react'
import Link from 'next/link'

export default () => {
  return (
    <IconContext.Provider
      value={{
        size: "1em",
        weight: "bold",
        className: "inline mb-1",
      }}
    >
      <header>
        <nav className="container mx-auto flex flex-row text-lg">
          <h1 className="text-xl py-2"><a href="/">Beacon explorer</a></h1>
          <ol className="flex flex-row flex-grow justify-end">
            <li className="p-2"><Link href="/epochs"><ClockCountdown /> Epochs</Link></li>
            <li className="p-2"><Link href="/blocks"><Cube /> Blocks</Link></li>
            <li className="p-2"><Link href="/validators"><UsersThree /> Validators</Link></li>
          </ol>
        </nav>
      </header>
    </IconContext.Provider>
  )
}