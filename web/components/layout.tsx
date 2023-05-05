import Header from "./header";

export default ({ children }) => {
  return (
    <>
      <Header />
      <div className="container mx-auto">
        {children}
      </div>
    </>
  )
}
