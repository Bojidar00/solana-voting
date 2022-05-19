<template>
  <wallet-multi-button></wallet-multi-button>
  <form>
   Topic: <input type="text" v-model="topic"/>
   Submition period: <input type="text" v-model="sdeadline"/>
   Voting period: <input type="text" v-model="vdeadline"/>
   Organisation PublicKey: <input type="text" v-model="org"/>
     
    </form>
    <button @click="create">create</button>
    <button @click="createwith">create with organisation</button>
    
    <form>
   Option: <input type="text" v-model="option"/>
     
    </form>
     
    <form>
      Organisation: <input type="text" v-model="organisation"/>
    </form>
    <button @click="createOrganisation">create organisation</button>





     <table>
       <tr><th>Organisation</th></tr>
       <tr v-for="org in organisations" :key="org.publicKey"><td>{{org.account.name}} {{org.publicKey}}</td><td><button @click="joinOrganisation(org)">join Organisation</button></td> <td>

           <form>
      Allow votiong: <input type="text" v-model="publicKey"/>
    </form>
    <button @click="allowVoting(org,publicKey)"> Allow votiong</button>
       </td></tr>
     </table>

     


   <table>
       <tr><th>Topic</th></tr>
       <tr v-for="topic in topics" :key="topic.publicKey"><td>{{topic.account.topic}}</td><td><button @click="addOption(topic)">add option</button></td></tr>
     </table>

     <table v-for="(topic, index) in topics" :key="topic.publicKey">
    
        <tr><th>{{topic.account.topic}}</th></tr>
        <tr v-for="option in options[index]" :key="option.id"><td>option:</td><td>{{option.name}}</td><td>{{option.votes}}</td><td><button @click="vote(topic,option.id)">vote</button></td></tr>
     </table>

     
    
</template>

<script>
import { WalletMultiButton } from 'solana-wallets-vue'
import { web3 } from '@project-serum/anchor'
//import * as anchor from '@project-serum/anchor';
import {  useWorkspace } from '../useWorkspace'
import idl from '../../../target/idl/voting.json'
import { PublicKey } from '@solana/web3.js'



const baseAccount = web3.Keypair.generate();
export default {
  name: 'HelloWorld',
  props: {
    msg: String
  },
  data(){
    return{
      topic:'',
      option:'',
      options:[],
      option1:'',
      option2:'',
      topics:[],
      sdeadline:0,
      vdeadline:0,
      organisation:'',
      organisations:[],
      publicKey:'',
      org:'',
    }
  },async mounted(){
    const { program } = useWorkspace()
     const data = await program.value.account.voteTopic.all();
     this.topics = data;
      

      for (let i =0;i<data.length;i++){
        let options= await this.getData(data[i]);
        this.options.push(options);
      }
      
    
      
      const data2 = await program.value.account.organisation.all();
     
      this.organisations=data2;
        const data3 = await program.value.account.organisationParticipant.all();
      console.log(data3);
  },
  components:{
    WalletMultiButton
  },
  methods:{
  async create(){
    console.log(this.topic);
    console.log(this.sdeadline);
    console.log(this.vdeadline);
      const {wallet, program } = useWorkspace()
      

      await program.value.rpc.create( this.topic,this.sdeadline,this.vdeadline,{
      accounts: {
        voteAccount: baseAccount.publicKey,
        user: wallet.value.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [baseAccount],
    });
     
   },
   async createwith(){
    console.log(this.topic);
    console.log(this.sdeadline);
    console.log(this.vdeadline);
      const {wallet, program } = useWorkspace()
      
      const pkey = new PublicKey(this.org);
      await program.value.rpc.createWithOrganisation( this.topic,this.sdeadline,this.vdeadline,pkey,{
      accounts: {
        voteAccount: baseAccount.publicKey,
        user: wallet.value.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [baseAccount],
    });
     
   },
   
  async  createOrganisation(){
     const {wallet, program } = useWorkspace()
     await program.value.rpc.createOrganisation( this.organisation,{
      accounts: {
        organisationAccount: baseAccount.publicKey,
        user: wallet.value.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [baseAccount],
    });
   },
   async joinOrganisation(org){
       const { program ,wallet } = useWorkspace()
      const programID = new PublicKey(idl.metadata.address);

      const [participantAccount, seed2] = await web3.PublicKey
      .findProgramAddress(
        [
         
          org.publicKey.toBuffer(),
         wallet.value.publicKey.toBuffer(),
        ],
        programID
      );
      console.log(seed2);

     await program.value.rpc.joinOrganisation({
      accounts: {
        organisationAccount: org.publicKey,
        organisationParticipant: participantAccount,
        user: wallet.value.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
     
    });
   },
   async allowVoting(org, publicKey){
const { program ,wallet } = useWorkspace()
      const programID = new PublicKey(idl.metadata.address);
       const pkey = new PublicKey(publicKey);

      const [participantAccount, seed2] = await web3.PublicKey
      .findProgramAddress(
        [
         
          org.publicKey.toBuffer(),
          pkey.toBuffer(),
        ],
        programID
      );
      console.log(seed2);

     await program.value.rpc.allowVoting({
      accounts: {
        organisationAccount: org.publicKey,
        participant: participantAccount,
        authority: wallet.value.publicKey,
      },
     
    });
   },
   async show(){
     const { program } = useWorkspace()
     const data = await program.value.account.topic.all();
     this.topics = data;
      console.log(data);
   },
   async vote(acc, num){
   
    console.log(acc);
     console.log(num);

    const { program ,wallet } = useWorkspace()
      const programID = new PublicKey(idl.metadata.address);

      const [voterAccount, seed2] = await web3.PublicKey
      .findProgramAddress(
        [
          //anchor.utils.bytes.utf8.encode(acc.account.topic),
          acc.publicKey.toBuffer(),
         wallet.value.publicKey.toBuffer(),
        ],
        programID
      );
      console.log(seed2);
    const [optionAccount, seed] = await web3.PublicKey
      .findProgramAddress(
        [
          acc.publicKey.toBuffer(),
          //anchor.utils.bytes.utf8.encode(acc.account.topic),
          [num],
        ],
        programID
      );
      console.log(seed);
      if(acc.account.use_organisation==false){
      await program.value.rpc.vote({
      accounts: {
        voteAccount: acc.publicKey,
        optionAccount:optionAccount,
        voterAccount:voterAccount,
         user: wallet.value.publicKey,
         systemProgram: web3.SystemProgram.programId,
      },
    });  }else{
      const [participantAccount, seed] = await web3.PublicKey
      .findProgramAddress(
        [
          acc.account.organisation.toBuffer(),
          //anchor.utils.bytes.utf8.encode(acc.account.topic),
          wallet.value.publicKey.toBuffer(),
        ],
        programID
      );
       console.log(seed);
       await program.value.rpc.voteWithOrganisation({
      accounts: {
        voteAccount: acc.publicKey,
        optionAccount:optionAccount,
        voterAccount:voterAccount,
        organisationParticipant:participantAccount,
         user: wallet.value.publicKey,
         systemProgram: web3.SystemProgram.programId,
      },
    });
    }
   },
   async addOption(acc){
     console.log(acc);
     const {wallet ,program} = useWorkspace();
     const programID = new PublicKey(idl.metadata.address)
     const [optionAccount, seed] = await web3.PublicKey
      .findProgramAddress(
        [
          //anchor.utils.bytes.utf8.encode(acc.account.topic),
          acc.publicKey.toBuffer(),
          [acc.account.optionsCount +1],
        ],
        programID
      );
      console.log(wallet);
    console.log("key: "+acc.publicKey);
    console.log(seed);
    console.log(optionAccount);
     
      await program.value.rpc.addOption( this.option,{
      accounts: {
        voteAccount: acc.publicKey,
        optionAccount: optionAccount,
        user: wallet.value.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
    });  
   },
   async getData(topic){
     const { program } = useWorkspace()
      const programID = new PublicKey(idl.metadata.address);
      let options=[];
if(topic.account.topic!='test' && topic.account.topic!='test2' && topic.account.topic!='test3' && topic.account.topic!='new topic'){
    
     for(let i =1;i<=topic.account.optionsCount;i++){
        
       const [optionAccount, seed] = await web3.PublicKey
      .findProgramAddress(
        [
          topic.publicKey.toBuffer(),
          //anchor.utils.bytes.utf8.encode(topic.account.topic),
          [i],
        ],
        programID
      );
       
      let option= await program.value.account.voteOption.fetch(optionAccount);
      options.push(option);
      console.log(seed);
      console.log( option);
     }
}
     return  options;
   }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
 table,th,td {
   border-collapse: collapse;
  border: 2px solid;
}
</style>
