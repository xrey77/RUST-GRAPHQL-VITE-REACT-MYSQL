import { gql } from '@apollo/client';

export const LIST_QUERY = gql`
  query ProductReport {
    productReport{
      id
      category
      descriptions
      qty
      unit
      costprice
      sellprice
      saleprice
      productpicture
      alertstocks
      criticalstocks
    }
  }
`;


export interface ProductData {
    id: number
    category: string
    descriptions: string
    qty: number
    unit: string
    costprice: number
    sellprice: number
    saleprice: number
    productpicture: string
    alertstocks: number
    criticalstocks: number
}

export interface ProductReportData {
    productReport: ProductData[]
}


