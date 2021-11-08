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
      total: (data) => parseInt(wrapper.dataset.pageCount!) * 10
    },
    pagination: {
      enabled: true,
      summary: false,
      limit: 10,
      server: {
        url: (prev, page, limit) => `${prev}?page=${page + 1}`
      }
    }
  }).render(wrapper);
}

