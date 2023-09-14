import './style.css'
import init from "wasm_bind_gen_block_on"

async function await_for(){
  await new Promise((resolve)=>{
    setTimeout(()=>{
      resolve();
    },3*1000)
  })
}

globalThis.await_for = await_for;


init("./public/wasm_bind_gen_block_on_bg.wasm").then((res)=>{
  console.log("init sucessfult",res)
  res.test()
})