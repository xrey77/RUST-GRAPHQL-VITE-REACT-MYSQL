import { gql } from '@apollo/client';
  
export const SALES_QUERY = gql`
  query SaleList {
    saleList{
      salesamount
      salesdate
    }
  }
`;

export interface SaleData {
    salesamount: number
    salesdate: string | number
}


export interface SalesListData {
    saleList: SaleData[]  
}






