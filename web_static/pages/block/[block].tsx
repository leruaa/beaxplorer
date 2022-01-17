import { useRouter } from 'next/router'
import Breadcrumb from "../../components/breadcrumb";
import { Blocks } from "../../pkg";

export async function getServerSideProps(context) {
  const wasmModule = await import('../../pkg');
  const blocks = await Blocks.build("http://localhost:3000");
  return {
    props: await blocks.get(context.params.block)
  }
}

export default (props) => {
  const router = useRouter()
  const { block } = router.query

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Blocks", icon: "clock" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <p>Showing epoch</p>

          <dl>
            <dt>Epoch</dt>
            <dd>{props.epoch}</dd>
            <dt>Slot</dt>
            <dd>{props.slot}</dd>
          </dl>
        </div>
      </section>
    </>
  )

}