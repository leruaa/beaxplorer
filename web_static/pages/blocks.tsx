import { useMemo, useCallback, useState } from "react";
import { useRouter } from 'next/router'
import moment from "moment";
import Moment from 'react-moment';
import DataTable from "../components/data-table";
import Number from "../components/number";
import Ethers from "../components/ethers";
import Percentage from "../components/percentage";
import Breadcrumb from "../components/breadcrumb";
import { Blocks } from "../pkg";


export async function getServerSideProps(context) {
  const blocks = await Blocks.build("http://localhost:3000");
  const pageIndex = parseInt(context.query.page, 10) - 1;
  return {
    props: {
      blocks: await blocks.page(pageIndex || 0, 10, "default", false),
      pageIndex
    }
  }
}

export default (props) => {

  const router = useRouter()
  const { page } = router.query
  const blocksMemo = useMemo(async () => await Blocks.build("http://localhost:3000"), []);

  const columns = [
    {
      accessor: "epoch",
      Header: "Epoch",
      Cell: ({ value }) => <a href={`/block/${value}`}><Number value={value} /></a>
    },
    {
      accessor: "slot",
      Header: "Block",
      Cell: ({ value }) => <a href={`/block/${value}`}><Number value={value} /></a>
    },
    {
      accessor: "status",
      Header: "Status"
    },
    {
      accessor: "proposer",
      Header: "Proposer"
    },
    {
      accessor: "attestations_count",
      Header: "Attestations",
      Cell: ({ value }) => <Number value={value} />
    },
    {
      accessor: "deposits_count",
      Header: "Deposits",
      Cell: ({ value }) => <Number value={value} />
    },
    {
      accessor: (row, rowIndex) => { return { p: row.proposer_slashings_count, a: row.attester_slashings_count } },
      Header: "Slashings P / A",
      Cell: ({ value }) =>
        <>
          <Number value={value.p} /> / <Number value={value.a} />
        </>
    },
    {
      accessor: "voluntary_exits_count",
      Header: "Exits",
      Cell: ({ value }) => <Number value={value} />
    }
  ];

  const getBlocksCount = useMemo(
    async (): Promise<number> => {
      const blocks = await blocksMemo;
      const meta = await blocks.meta();
      return meta.count;
    },
    [],
  );

  const [pageCount, setPageCount] = useState(100);
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  const fetchData = useCallback(async ({ pageSize, pageIndex, sortBy }) => {
    if (pageIndex == props.pageIndex) {
      setData(props.blocks);
    }
    else {
      const blocks = await blocksMemo;
      let sortId = sortBy.length > 0 ? sortBy[0].id : "slot";
      let sortDesc = sortBy.length > 0 ? sortBy[0].desc : false;

      if (["epoch", "slot", "timestamp"].indexOf(sortId) > -1) sortId = "default";

      setData(await blocks.page(
        pageIndex,
        pageSize,
        sortId,
        sortDesc
      ));
      setPageCount(Math.ceil(await getBlocksCount / pageSize));
    }
  }, []);

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Blocks", icon: "cube" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <DataTable
            columns={useMemo(() => columns, [])}
            data={data}
            fetchData={fetchData}
            loading={loading}
            pageIndex={page ? parseInt(page as string, 10) - 1 : 0}
            pageCount={pageCount}
            sortBy={useMemo(() => [{ id: "slot", desc: false }], [])}
          />
        </div>
      </section>
    </>
  )
}
