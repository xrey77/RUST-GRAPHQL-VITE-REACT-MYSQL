import { gql } from '@apollo/client';

  // query SearchProduct($page: Int!, $keyword: String!) {
  // searchProducts{
  //   searchProduct(page: $page, keyword: $keyword) {
  //     page
  //     totpage
  //     totalrecords
  //     products {
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
  //   }
  // }
  // }

export const SEARCH_QUERY = gql`
  query ProductSearch($page: Int!, $keyword: String!) {
    productSearch(page: $page, keyword: $keyword) {
      products{
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
      totalPages
      totalRecords
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

export interface ProductSearchData {
    productSearch: {
      page: number;
      totalPages: number;
      totalRecords: number;
      products: ProductData[];
    }
}

export interface ProductSearchVariables {
    keyword: string;
    page: number;
}


