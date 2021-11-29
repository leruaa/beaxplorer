import Icon from "./icon";

export default () => {
  return (
    <nav className="container mx-auto flex flex-row text-lg">
      <h1 className="text-xl py-2"><a href="/">Beacon explorer</a></h1>
      <ol className="flex flex-row flex-grow justify-end">
        <li className="p-2"><a href="/epochs"><i className="icon outline-clock" /> Epochs</a></li>
        <li className="p-2"><a href="/blocks"><i className="icon outline-cube" /> Blocks</a></li>
        <li className="p-2"><a href="/validators"><i className="icon outline-users" /> Validators</a></li>
      </ol>
    </nav>
  )
}