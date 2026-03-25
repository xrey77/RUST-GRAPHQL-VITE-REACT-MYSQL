import Footer from "./Footer.tsx";

export default function About() {
  return (
    <>
<div className="card bg-dark border-0 mt-3 about">
  <img src="/assets/images/about.jpg" className="card-img " alt="..."/>
  <div className="card-body">
  </div>
  <div className="card-footer">
  <h2 className="card-title text-white card-x embossed">About Us</h2>
    <p className="card-text text-white card-x">
   The World Bank Group is a specialized United Nations agency established in 1944 at the Bretton Woods Conference to reduce global poverty and boost shared prosperity. It provides financing, technical advice, and policy guidance to low- and middle-income countries. Comprising 189 member countries, it functions as a partnership for sustainable development, focusing on projects in health, education, and climate change.
      </p>
    <p className="card-text"><small>Last updated 3 mins ago</small></p>

  </div>
</div> 
<div className="w-80 fixed-bottom">   
<Footer/>
</div>
</>
  )
}
