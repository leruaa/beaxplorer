import { useMemo, useCallback, useState } from "react";
import { useRouter } from 'next/router'
import moment from "moment";
import Moment from 'react-moment';
import DataTable from "../components/data-table";
import Ethers from "../components/ethers";
import Percentage from "../components/percentage";
import Breadcrumb from "../components/breadcrumb";
import { Epochs, SortBy } from "../pkg";


export async function getServerSideProps(context) {
  const epochs = await Epochs.build("http://localhost:3000");
  const pageIndex = parseInt(context.query.page, 10) - 1;
  return {
    props: {
      epochs: await epochs.page(pageIndex || 0, 10),
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
      Cell: ({ value }) => <a href={`/epoch/${value}`}>{value}</a>
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
      Header: "Attestations"
    },
    {
      accessor: "deposits_count",
      Header: "Deposits",
    },
    {
      accessor: (row, rowIndex) => `${row.proposer_slashings_count} / ${row.attester_slashings_count}`,
      Header: "Slashings P / A",
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
      setData(
        await epochs.page(
          pageIndex,
          pageSize,
          sortBy.length == 0 ? null : new SortBy(sortBy[0].id, sortBy[0].desc)
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
          />
        </div>
      </section>
    </>
  )
}
  