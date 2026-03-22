import { gql } from '@apollo/client';
  
export const SALES_QUERY = gql`
  query SalesList {
    salesList{
      id
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
    salesList: SaleData[]  
}






