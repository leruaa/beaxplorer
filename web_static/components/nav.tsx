import Icon from "./icon";

export default () => {
  return (
    <nav className="container mx-auto flex flex-row text-lg">
      <h1 className="text-xl py-2"><a href="/">Beacon explorer</a></h1>
      <ol className="flex flex-row flex-grow justify-end">
        <li className="p-2"><a href="/epochs"><Icon v="outline" id="clock" /> Epochs</a></li>
        <li className="p-2"><a href="/blocks"><Icon v="outline" id="cube" /> Blocks</a></li>
        <li className="p-2"><a href="/validators"><Icon v="outline" id="users" /> Validators</a></li>
      </ol>
    </nav>
  )
}