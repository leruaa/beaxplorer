import Link from 'next/link'

export default () => {
  return (
    <>
      <header>
        <nav className="container mx-auto flex flex-row text-lg">
          <h1 className="text-xl py-2"><a href="/">Beacon explorer</a></h1>
          <ol className="flex flex-row flex-grow justify-end">
            <li className="p-2"><Link href="/epochs"><a><i className="icon outline-clock" /> Epochs</a></Link></li>
            <li className="p-2"><Link href="/blocks"><a><i className="icon outline-cube" /> Blocks</a></Link></li>
            <li className="p-2"><Link href="/validators"><a><i className="icon outline-users" /> Validators</a></Link></li>
          </ol>
        </nav>
      </header>
    </>
  )
}