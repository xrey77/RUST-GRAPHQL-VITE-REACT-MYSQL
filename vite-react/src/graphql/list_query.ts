import { gql } from '@apollo/client';

  // query ProductList($page: Int!) {
  //   productList(page: $page) {
  //     products{
  //       id
  //       category
  //       descriptions
  //       qty
  //       unit
  //       costprice
  //       sellprice
  //       saleprice
  //       productpicture
  //       alertstocks
  //       criticalstocks
  //     }
  //     page
  //     totalPages
  //     totalRecords    
  //   }
  // }

  
export const LIST_QUERY = gql`
  query ProductList($page: Int!) {
    productList(page: $page) {
      products {
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
      page
      totpage
      totalrecords
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

export interface ProductListData {
    productList: {
      page: number;
      totpage: number;
      totalrecords: number;
      products: ProductData[];
    }
}

export interface ProductListVariables {
  page: number;
}


