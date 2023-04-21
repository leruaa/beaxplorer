import Link from 'next/link'

export default () => {
  return (
    <>
      <header>
        <nav className="container mx-auto flex flex-row text-lg">
          <h1 className="text-xl py-2"><a href="/">Beacon explorer</a></h1>
          <ol className="flex flex-row flex-grow justify-end">
            <li className="p-2"><Link href="/epochs"><i className="icon outline-clock" /> Epochs</Link></li>
            <li className="p-2"><Link href="/blocks"><i className="icon outline-cube" /> Blocks</Link></li>
            <li className="p-2"><Link href="/validators"><i className="icon outline-users" /> Validators</Link></li>
          </ol>
        </nav>
      </header>
    </>
  )
}