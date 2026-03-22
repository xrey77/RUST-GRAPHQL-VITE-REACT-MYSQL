import { gql } from '@apollo/client';

  // query {
  //   productsCategory {
  //     name 
  //     products {
  //       id
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


export const PRODUCT_CATEGORY_QUERY = gql`
  query ProductCategory{
    productCategories{
      name
      products {
        id
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
  }
`;



export interface Product {
  id: string;
  descriptions: string;
  qty: number;
  unit: string;
  costprice: number;
  sellprice: number;
  saleprice: number;
  productpicture: string;
  alertstocks: number;
  criticalstocks: number;
}

export interface Category {
  name: string;
  products: Product[];
}

export interface ProductCategoriesData {
  productCategories: Category[];
}




