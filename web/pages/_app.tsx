import Layout from '../components/layout';
import '../styles/main.css'

export default ({ Component, pageProps }) => {
  return (
    <Layout>
      <Component {...pageProps} />
    </Layout>
  )
}