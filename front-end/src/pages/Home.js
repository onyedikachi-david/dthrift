import { Link } from "react-router-dom";
import logoWallet from "../assets/logo.png";
import { AiOutlineDoubleRight } from "react-icons/ai";
import { SiEthereum, SiBitcoin, SiLitecoin } from "react-icons/si";
import "../styles/home.css";
const Home = () => {
  return (
    <div className="home-page">
      <SiEthereum className="crypto-icon eth" />
      <header className="home-page-header">
        <div className="home-logo-container">
          <img src={logoWallet} alt="logo" className="home-logo" />
        </div>
        <Link className="launch-app-link" to="/app/purses">
          Launch App!
        </Link>
      </header>
      <SiLitecoin className="crypto-icon ltc" />
      <main className="home-main">
        <p className="first-para">NO TRUST REQUIRED</p>
        <h1 className="main-header">
          Here it is, your truly Decentralized Thrift
        </h1>
        <p className="second-para">
          Create or join already created
          <br />
          purse to quickly and easily meet your financial target
        </p>
        <Link className="launch-app-link-btn" to="/app/purses">
          Launch App <AiOutlineDoubleRight className="launch-app-icon" />
        </Link>
      </main>
      <SiBitcoin className="crypto-icon btc" />
    </div>
  );
};

export default Home;
