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
      <header className="p-1 bg-blue-900 text-blue-50">
        <nav className="container mx-auto flex flex-row text-lg text-blue-50">
          <h1 className="text-xl py-2"><Link className="text-blue-50" href="/">Beacon explorer</Link></h1>
          <ol className="flex flex-row flex-grow justify-end">
            <li className="p-2"><Link href="/epochs" className="text-blue-50"><ClockCountdown /> Epochs</Link></li>
            <li className="p-2"><Link href="/blocks" className="text-blue-50"><Cube /> Blocks</Link></li>
            <li className="p-2"><Link href="/validators" className="text-blue-50"><UsersThree /> Validators</Link></li>
          </ol>
        </nav>
      </header>
    </IconContext.Provider>
  )
}