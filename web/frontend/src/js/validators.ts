import { html } from "gridjs";
import { paginate } from "./pagination";

paginate({
  wrapperId: "validators-list",
  apiUrl: "/api/validators",
  columns: [
    {
      id: "index",
      name: "Index",
      formatter: (cell: any) => html(`<a href="/validator/${cell}">${cell}</a>`)
    },
    "Public key",
    {
      id: "balance",
      name: "Balance"
    },
    {
      id: "status",
      name: "Status"
    },
    {
      id: "activation_ago",
      name: "Activation"
    },
    {
      id: "exit_ago",
      name: "Exit",
      formatter: (cell: any) => cell ? cell : html("&ndash;")
    },
    {
      id: "withdrawable_ago",
      name: "Withdrawable epoch",
      formatter: (cell: any) => cell ? cell : html("&ndash;")
    }
  ],
  dataMapping: (data: any) => data.results.map(
    (e: any) => [
      e.index,
      e.public_key_extract,
      `${e.balance} ETH (${e.effective_balance} ETH)`,
      e.status,
      `${e.activation_ago} (${e.activation_epoch})`,
      e.exit_ago ? `${e.exit_ago} (${e.exit_epoch})` : null,
      e.withdrawable_ago ? `${e.withdrawable_ago} (${e.withdrawable_epoch})` : null
    ]
  )
});
