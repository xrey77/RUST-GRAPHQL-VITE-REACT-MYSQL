import { useLazyQuery } from '@apollo/client/react';
import { PRODUCT_CATEGORY_QUERY } from '../graphql/productCategory_query.ts';
import type { Category, ProductCategoriesData } from '../graphql/productCategory_query.ts';
import { useEffect, useState } from 'react';
import { InventoryReport } from './CategoryTemplate.tsx';
import { PDFViewer } from '@react-pdf/renderer';
import { useWindowSize } from '../useWindowSize.ts';

export default function ProductCategory() {
    const [message, setMessage] = useState<string>('');
    const [categoryData, setCategoryData] = useState<Category[]>([]);

    const [products] = useLazyQuery<ProductCategoriesData>(PRODUCT_CATEGORY_QUERY);

    const getCategories = async () => {
        setMessage("Loading inventory data...");
        try {
            const { data } = await products();
            if (data?.productCategories) {              
                setCategoryData(data.productCategories);
            }                

        } catch (err: any) {  
            if (err.AbortError) {
                setMessage(err.message);
            }
            setTimeout(() => { setMessage('');  }, 1000);
        }

    }
    const { width, height } = useWindowSize();
    useEffect(() => {
        getCategories()
        
    },[])

  return (
  <div className="container-fluid">

{categoryData ? (
  
  <div className="container-fluid"> 
    <PDFViewer width={width-50} height={height}>
      <InventoryReport data={{ productCategories: categoryData }} />
    </PDFViewer>
    <br/><br/><br/><br/>
  </div>
) : (
  <p>{message}</p>
)}    
  </div>
  )
}
