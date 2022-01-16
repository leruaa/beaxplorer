import { useMemo, useCallback, useState } from "react";
import { useRouter } from 'next/router'
import moment from "moment";
import Moment from 'react-moment';
import DataTable from "../components/data-table";
import Number from "../components/number";
import Ethers from "../components/ethers";
import Percentage from "../components/percentage";
import Breadcrumb from "../components/breadcrumb";
import { Epochs } from "../pkg";


export async function getServerSideProps(context) {
  const epochs = await Epochs.build("http://localhost:3000");
  const pageIndex = parseInt(context.query.page, 10) - 1;
  return {
    props: {
      epochs: await epochs.page(pageIndex || 0, 10, "default", false),
      pageIndex
    }
  }
}

export default (props) => {

  const router = useRouter()
  const { page } = router.query
  const epochsMemo = useMemo(async () => await Epochs.build("http://localhost:3000"), []);

  const columns = [
    {
      accessor: "epoch",
      Header: "Epoch",
      Cell: ({ value }) => <a href={`/epoch/${value}`}><Number value={value} /></a>
    },
    {
      accessor: "timestamp",
      Header: "Time",
      Cell: ({ value }) =>
        <span title={moment.unix(value).format("L LTS")}>
          <Moment unix fromNow date={value} />
        </span>
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
      accessor: "finalized",
      Header: "Finalized",
      Cell: ({ value }) => value ? "Yes" : "No"
    },
    {
      accessor: "eligible_ether",
      Header: "Eligible",
      Cell: ({ value }) => <Ethers value={value} />
    },
    {
      accessor: "voted_ether",
      Header: "Voted",
      Cell: ({ value }) => <Ethers value={value} />
    },
    {
      accessor: "global_participation_rate",
      Header: "Rate",
      Cell: ({ value }) => <Percentage value={value} />
    }
  ];

  const getEpochsCount = useMemo(
    async (): Promise<number> => {
      const epochs = await epochsMemo;
      const meta = await epochs.meta();
      return meta.count;
    },
    [],
  );

  const [pageCount, setPageCount] = useState(100);
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  const fetchData = useCallback(async ({ pageSize, pageIndex, sortBy }) => {
    if (pageIndex == props.pageIndex) {
      setData(props.epochs);
    }
    else {
      const epochs = await epochsMemo;
      let sortId = sortBy.length > 0 ? sortBy[0].id : "epoch";
      let sortDesc = sortBy.length > 0 ? sortBy[0].desc : false;

      if (["epoch", "timestamp"].indexOf(sortId) > -1) sortId = "default";

      setData(
        await epochs.page(
          pageIndex,
          pageSize,
          sortId,
          sortDesc
        )
      );
      setPageCount(Math.ceil(await getEpochsCount / pageSize));
    }
  }, []);

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Epochs", icon: "clock" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <DataTable
            columns={useMemo(() => columns, [])}
            data={data}
            fetchData={fetchData}
            loading={loading}
            pageIndex={page ? parseInt(page as string, 10) - 1 : 0}
            pageCount={pageCount}
            sortBy={useMemo(() => [{ id: "epoch", desc: false }], [])}
          />
        </div>
      </section>
    </>
  )
}
