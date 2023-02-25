import {Switch, Route} from "react-router-dom"
import Home from "../pages/Home"
const HomeView = () => {
    return(
        <div className = "home-view">
            <Switch>
                <Route exact path = "/">
                    <Home />
                </Route>
            </Switch>
        </div>
    );
}

export default HomeView;