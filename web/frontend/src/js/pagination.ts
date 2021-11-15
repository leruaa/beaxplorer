import { Grid } from "gridjs";

export interface PaginationSettings {
  wrapperId: string,
  columns: any[],
  apiUrl: string,
  dataMapping: (data: any) => any[][]
}

export function paginate(settings: PaginationSettings) {
  let wrapper = document.getElementById(settings.wrapperId)!;

  new Grid({
    columns: settings.columns,
    server: {
      url: settings.apiUrl,
      then: settings.dataMapping,
      total: (data) => data.page_count * 10
    },
    pagination: {
      enabled: true,
      summary: false,
      limit: 10,
      server: {

        url: (prev, page, limit) => {
          let sep = (prev.indexOf("?") > -1 ? "&" : "?");
          return `${prev}${sep}page=${page + 1}`
        }
      }
    },
    sort: {
      multiColumn: false,
      server: {
        url: (prev, columns) => {
          if (!columns.length) return prev;

          const col = columns[0];
          const dir = col.direction === 1 ? 'asc' : 'desc';
          let colName = settings.columns[col.index].id;
          let sep = (prev.indexOf("?") > -1 ? "&" : "?");

          return `${prev}${sep}sort=${colName}&dir=${dir}`;
        }
      }
    },
  }).render(wrapper);
}

