import Footer from "./Footer.tsx";

export default function Contact() {
  return (
    <>
    <div className="contact-bg">
      <div className="card border-0 mb-3 mt-5 contact">
        <div className="row g-0">
          <div className="col-md-4">
            <img src="./images/contact.jpeg" className="card-h rounded-start" alt="..."/>
          </div>
          <div className="col-md-8">
            <div className="card-body">
              <h2 className="card-title text-white embossed">Contact Us</h2>
              <p className="card-text text-white txt-justified">
              IBRD & IDA (The World Bank): The International Bank for Reconstruction and Development (IBRD) lends to governments of middle-income countries, while the International Development Association (IDA) provides interest-free loans and grants to the poorest nations. 
              <br/><br/>
              Private Sector Focus: The International Finance Corporation (IFC) and Multilateral Investment Guarantee Agency (MIGA) support private enterprise and provide risk insurance to encourage investment.
                </p>
              <p className="card-text text-white"><small className="text-body-secondary text-white">
              <strong className="text-white">Headquarters</strong> <br/>
              <div className="text-white">
              The World Bank. 1818 H Street, NW Washington, DC 20433 USA. Tel : (202) 473-1000. info@worldbank.org. ASK THE BANK. Useful Links: Jobs and Internships 
              </div>
              </small></p>
            </div>
          </div>
        </div>
      </div>   
      <div className="footer-contact"> 
        <Footer/>
      </div>
      </div>      
</>
  )
}
